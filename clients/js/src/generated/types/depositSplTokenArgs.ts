/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, bool, struct } from '@metaplex-foundation/umi/serializers';

export type DepositSplTokenArgs = { reversed: boolean };

export type DepositSplTokenArgsArgs = DepositSplTokenArgs;

export function getDepositSplTokenArgsSerializer(): Serializer<
  DepositSplTokenArgsArgs,
  DepositSplTokenArgs
> {
  return struct<DepositSplTokenArgs>([['reversed', bool()]], {
    description: 'DepositSplTokenArgs',
  }) as Serializer<DepositSplTokenArgsArgs, DepositSplTokenArgs>;
}
