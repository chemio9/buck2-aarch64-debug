/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

package com.facebook.buck.jvm.java.abi.kotlin;

import com.google.common.collect.ImmutableList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Iterator;
import java.util.List;
import java.util.Objects;
import java.util.stream.Stream;
import kotlin.Metadata;
import kotlin.metadata.Attributes;
import kotlin.metadata.KmDeclarationContainer;
import kotlin.metadata.KmProperty;
import kotlin.metadata.jvm.JvmExtensionsKt;
import kotlin.metadata.jvm.JvmMetadataUtil;
import kotlin.metadata.jvm.JvmMethodSignature;
import kotlin.metadata.jvm.KotlinClassMetadata;
import org.objectweb.asm.tree.AnnotationNode;

/** Utilities to read Kotlin class metadata. */
public class KotlinMetadataReader {

  /**
   * Method to find the inline functions of a Kotlin class by finding the Kotlin metadata annotation
   * and reading it.
   */
  public static ImmutableList<String> getInlineFunctions(AnnotationNode annotationNode) {
    Metadata classHeader = createHeader(annotationNode);
    KotlinClassMetadata metadata = KotlinClassMetadata.readStrict(classHeader);
    if (metadata == null) {
      throw new AssertionError(
          "Unsupported kind of Kotlin classes: ["
              + classHeader.k()
              + "] or has an unsupported metadata version: ["
              + Arrays.toString(classHeader.mv())
              + "]");
    }

    KmDeclarationContainer container;
    if (metadata instanceof KotlinClassMetadata.Class) {
      container = ((KotlinClassMetadata.Class) metadata).getKmClass();
    } else if (metadata instanceof KotlinClassMetadata.FileFacade) {
      container = ((KotlinClassMetadata.FileFacade) metadata).getKmPackage();
    } else if (metadata instanceof KotlinClassMetadata.MultiFileClassPart) {
      container = ((KotlinClassMetadata.MultiFileClassPart) metadata).getKmPackage();
    } else {
      return ImmutableList.of();
    }

    ImmutableList<String> inlineFunctions =
        container.getFunctions().stream()
            .filter(Attributes::isInline)
            .map(JvmExtensionsKt::getSignature)
            .filter(Objects::nonNull)
            .map(JvmMethodSignature::getName)
            .collect(ImmutableList.toImmutableList());

    ImmutableList<String> inlineProperties =
        container.getProperties().stream()
            .filter(
                it ->
                    Attributes.isInline(it.getGetter())
                        || it.getSetter() != null && Attributes.isInline(it.getSetter()))
            .map(KmProperty::getName)
            .collect(ImmutableList.toImmutableList());

    ImmutableList<String> inlineGetters =
        container.getProperties().stream()
            .filter(it -> Attributes.isInline(it.getGetter()))
            .map(JvmExtensionsKt::getGetterSignature)
            .filter(Objects::nonNull)
            .map(JvmMethodSignature::getName)
            .collect(ImmutableList.toImmutableList());

    ImmutableList<String> inlineSetters =
        container.getProperties().stream()
            .filter(it -> it.getSetter() != null && Attributes.isInline(it.getSetter()))
            .map(JvmExtensionsKt::getSetterSignature)
            .filter(Objects::nonNull)
            .map(JvmMethodSignature::getName)
            .collect(ImmutableList.toImmutableList());

    return Stream.of(inlineFunctions, inlineProperties, inlineGetters, inlineSetters)
        .flatMap(Collection::stream)
        .distinct()
        .sorted()
        .collect(ImmutableList.toImmutableList());
  }

  /**
   * Converts the given AnnotationNode representing the @kotlin.Metadata annotation into
   * KotlinClassHeader, to be able to use it in KotlinClassMetadata.read.
   */
  private static Metadata createHeader(AnnotationNode node) {
    Integer kind = null;
    int[] metadataVersion = null;
    String[] data1 = null;
    String[] data2 = null;
    String extraString = null;
    String packageName = null;
    Integer extraInt = null;

    Iterator<Object> it = node.values.iterator();
    while (it.hasNext()) {
      String name = (String) it.next();
      Object value = it.next();

      switch (name) {
        case "k":
          kind = (Integer) value;
          break;
        case "mv":
          metadataVersion = listToIntArray(value);
          break;
        case "d1":
          data1 = listToStringArray(value);
          break;
        case "d2":
          data2 = listToStringArray(value);
          break;
        case "xs":
          extraString = (String) value;
          break;
        case "pn":
          packageName = (String) value;
          break;
        case "xi":
          extraInt = (Integer) value;
          break;
      }
    }

    return JvmMetadataUtil.Metadata(
        kind, metadataVersion, data1, data2, extraString, packageName, extraInt);
  }

  @SuppressWarnings("unchecked")
  private static int[] listToIntArray(Object list) {
    return ((List<Integer>) list).stream().mapToInt(i -> i).toArray();
  }

  @SuppressWarnings("unchecked")
  private static String[] listToStringArray(Object list) {
    return ((List<String>) list).toArray(new String[0]);
  }
}
