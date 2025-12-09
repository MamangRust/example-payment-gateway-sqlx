import { z } from "zod";

const CardIdSchema = z.object({
  value: z.string(),
  label: z.string(),
});

const MerchantIdSchema = z.object({
  value: z.number(),
  label: z.string(),
});

export const updateTransactionRequestSchema = z.object({
  card_number: CardIdSchema,
  merchant_id: MerchantIdSchema,
  amount: z.number().min(1).max(100000000),
  payment_method: z.string().min(1).max(16),
  transaction_time: z.coerce.date(),
});

export type UpdateTransactionFormValues = z.infer<
  typeof updateTransactionRequestSchema
>;
