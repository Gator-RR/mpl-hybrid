/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  GetDataEnumKind,
  GetDataEnumKindContent,
  Serializer,
  dataEnum,
  publicKey as publicKeySerializer,
  struct,
  tuple,
  u64,
  unit,
} from '@metaplex-foundation/umi/serializers';

export type IngredientV1 =
  | { __kind: 'None' }
  | { __kind: 'Sol'; fields: [bigint] }
  | { __kind: 'CoreAsset'; fields: [PublicKey] }
  | { __kind: 'CoreCollection'; fields: [PublicKey] }
  | { __kind: 'SplToken'; fields: [PublicKey, bigint] }
  | { __kind: 'SplToken22'; fields: [PublicKey, bigint] }
  | { __kind: 'TmNft'; fields: [PublicKey] }
  | { __kind: 'TmNftCollection'; fields: [PublicKey] }
  | { __kind: 'TmPNft'; fields: [PublicKey] }
  | { __kind: 'TmPNftCollection'; fields: [PublicKey] }
  | { __kind: 'CompressedNft'; fields: [PublicKey] }
  | { __kind: 'CompressedNftCollection'; fields: [PublicKey] };

export type IngredientV1Args =
  | { __kind: 'None' }
  | { __kind: 'Sol'; fields: [number | bigint] }
  | { __kind: 'CoreAsset'; fields: [PublicKey] }
  | { __kind: 'CoreCollection'; fields: [PublicKey] }
  | { __kind: 'SplToken'; fields: [PublicKey, number | bigint] }
  | { __kind: 'SplToken22'; fields: [PublicKey, number | bigint] }
  | { __kind: 'TmNft'; fields: [PublicKey] }
  | { __kind: 'TmNftCollection'; fields: [PublicKey] }
  | { __kind: 'TmPNft'; fields: [PublicKey] }
  | { __kind: 'TmPNftCollection'; fields: [PublicKey] }
  | { __kind: 'CompressedNft'; fields: [PublicKey] }
  | { __kind: 'CompressedNftCollection'; fields: [PublicKey] };

export function getIngredientV1Serializer(): Serializer<
  IngredientV1Args,
  IngredientV1
> {
  return dataEnum<IngredientV1>(
    [
      ['None', unit()],
      [
        'Sol',
        struct<GetDataEnumKindContent<IngredientV1, 'Sol'>>([
          ['fields', tuple([u64()])],
        ]),
      ],
      [
        'CoreAsset',
        struct<GetDataEnumKindContent<IngredientV1, 'CoreAsset'>>([
          ['fields', tuple([publicKeySerializer()])],
        ]),
      ],
      [
        'CoreCollection',
        struct<GetDataEnumKindContent<IngredientV1, 'CoreCollection'>>([
          ['fields', tuple([publicKeySerializer()])],
        ]),
      ],
      [
        'SplToken',
        struct<GetDataEnumKindContent<IngredientV1, 'SplToken'>>([
          ['fields', tuple([publicKeySerializer(), u64()])],
        ]),
      ],
      [
        'SplToken22',
        struct<GetDataEnumKindContent<IngredientV1, 'SplToken22'>>([
          ['fields', tuple([publicKeySerializer(), u64()])],
        ]),
      ],
      [
        'TmNft',
        struct<GetDataEnumKindContent<IngredientV1, 'TmNft'>>([
          ['fields', tuple([publicKeySerializer()])],
        ]),
      ],
      [
        'TmNftCollection',
        struct<GetDataEnumKindContent<IngredientV1, 'TmNftCollection'>>([
          ['fields', tuple([publicKeySerializer()])],
        ]),
      ],
      [
        'TmPNft',
        struct<GetDataEnumKindContent<IngredientV1, 'TmPNft'>>([
          ['fields', tuple([publicKeySerializer()])],
        ]),
      ],
      [
        'TmPNftCollection',
        struct<GetDataEnumKindContent<IngredientV1, 'TmPNftCollection'>>([
          ['fields', tuple([publicKeySerializer()])],
        ]),
      ],
      [
        'CompressedNft',
        struct<GetDataEnumKindContent<IngredientV1, 'CompressedNft'>>([
          ['fields', tuple([publicKeySerializer()])],
        ]),
      ],
      [
        'CompressedNftCollection',
        struct<GetDataEnumKindContent<IngredientV1, 'CompressedNftCollection'>>(
          [['fields', tuple([publicKeySerializer()])]]
        ),
      ],
    ],
    { description: 'IngredientV1' }
  ) as Serializer<IngredientV1Args, IngredientV1>;
}

// Data Enum Helpers.
export function ingredientV1(
  kind: 'None'
): GetDataEnumKind<IngredientV1Args, 'None'>;
export function ingredientV1(
  kind: 'Sol',
  data: GetDataEnumKindContent<IngredientV1Args, 'Sol'>['fields']
): GetDataEnumKind<IngredientV1Args, 'Sol'>;
export function ingredientV1(
  kind: 'CoreAsset',
  data: GetDataEnumKindContent<IngredientV1Args, 'CoreAsset'>['fields']
): GetDataEnumKind<IngredientV1Args, 'CoreAsset'>;
export function ingredientV1(
  kind: 'CoreCollection',
  data: GetDataEnumKindContent<IngredientV1Args, 'CoreCollection'>['fields']
): GetDataEnumKind<IngredientV1Args, 'CoreCollection'>;
export function ingredientV1(
  kind: 'SplToken',
  data: GetDataEnumKindContent<IngredientV1Args, 'SplToken'>['fields']
): GetDataEnumKind<IngredientV1Args, 'SplToken'>;
export function ingredientV1(
  kind: 'SplToken22',
  data: GetDataEnumKindContent<IngredientV1Args, 'SplToken22'>['fields']
): GetDataEnumKind<IngredientV1Args, 'SplToken22'>;
export function ingredientV1(
  kind: 'TmNft',
  data: GetDataEnumKindContent<IngredientV1Args, 'TmNft'>['fields']
): GetDataEnumKind<IngredientV1Args, 'TmNft'>;
export function ingredientV1(
  kind: 'TmNftCollection',
  data: GetDataEnumKindContent<IngredientV1Args, 'TmNftCollection'>['fields']
): GetDataEnumKind<IngredientV1Args, 'TmNftCollection'>;
export function ingredientV1(
  kind: 'TmPNft',
  data: GetDataEnumKindContent<IngredientV1Args, 'TmPNft'>['fields']
): GetDataEnumKind<IngredientV1Args, 'TmPNft'>;
export function ingredientV1(
  kind: 'TmPNftCollection',
  data: GetDataEnumKindContent<IngredientV1Args, 'TmPNftCollection'>['fields']
): GetDataEnumKind<IngredientV1Args, 'TmPNftCollection'>;
export function ingredientV1(
  kind: 'CompressedNft',
  data: GetDataEnumKindContent<IngredientV1Args, 'CompressedNft'>['fields']
): GetDataEnumKind<IngredientV1Args, 'CompressedNft'>;
export function ingredientV1(
  kind: 'CompressedNftCollection',
  data: GetDataEnumKindContent<
    IngredientV1Args,
    'CompressedNftCollection'
  >['fields']
): GetDataEnumKind<IngredientV1Args, 'CompressedNftCollection'>;
export function ingredientV1<K extends IngredientV1Args['__kind']>(
  kind: K,
  data?: any
): Extract<IngredientV1Args, { __kind: K }> {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}
export function isIngredientV1<K extends IngredientV1['__kind']>(
  kind: K,
  value: IngredientV1
): value is IngredientV1 & { __kind: K } {
  return value.__kind === kind;
}
