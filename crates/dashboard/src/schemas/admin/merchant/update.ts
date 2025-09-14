import { z } from "zod";

const UserIdSchema = z.object({
  value: z.string(),
  label: z.string(),
});

export const updateMerchantRequestSchema = z.object({
  name: z.string(),
  user_id: UserIdSchema,
  status: z.string(),
});

export type UpdateMerchantFormValues = z.infer<
  typeof updateMerchantRequestSchema
>;
