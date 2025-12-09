import { z } from "zod";

export const createMerchantRequestSchema = z.object({
  name: z.string(),
  user_id: z.number().int().min(1),
});

export type CreateMerchantFormValues = z.infer<
  typeof createMerchantRequestSchema
>;
