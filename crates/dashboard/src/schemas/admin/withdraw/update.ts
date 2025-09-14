import { z } from "zod";

const CardIdSchema = z.object({
  value: z.string(),
  label: z.string(),
});

export const updateWithdrawRequestSchema = z.object({
  card_number: CardIdSchema,
  withdraw_amount: z.number().min(1).max(100000000),
  withdraw_time: z.coerce.date(),
});

export type UpdateWithdrawFormValues = z.infer<
  typeof updateWithdrawRequestSchema
>;
