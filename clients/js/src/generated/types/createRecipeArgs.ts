/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Serializer,
  array,
  bool,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  IngredientTriggerPairV1,
  IngredientTriggerPairV1Args,
  getIngredientTriggerPairV1Serializer,
} from '.';

export type CreateRecipeArgs = {
  reversible: boolean;
  inputs: Array<IngredientTriggerPairV1>;
  outputs: Array<IngredientTriggerPairV1>;
};

export type CreateRecipeArgsArgs = {
  reversible: boolean;
  inputs: Array<IngredientTriggerPairV1Args>;
  outputs: Array<IngredientTriggerPairV1Args>;
};

export function getCreateRecipeArgsSerializer(): Serializer<
  CreateRecipeArgsArgs,
  CreateRecipeArgs
> {
  return struct<CreateRecipeArgs>(
    [
      ['reversible', bool()],
      ['inputs', array(getIngredientTriggerPairV1Serializer())],
      ['outputs', array(getIngredientTriggerPairV1Serializer())],
    ],
    { description: 'CreateRecipeArgs' }
  ) as Serializer<CreateRecipeArgsArgs, CreateRecipeArgs>;
}