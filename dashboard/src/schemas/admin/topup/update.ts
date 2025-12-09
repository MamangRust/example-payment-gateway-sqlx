import { z } from "zod";

const CardIdSchema = z.object({
  value: z.string(),
  label: z.string(),
});

export const updateTopupRequestSchema = z.object({
  card_number: CardIdSchema,
  topup_amount: z.number().min(1).max(100000000),
  topup_method: z.string().min(1).max(16),
});

export type UpdateTopupFormValues = z.infer<typeof updateTopupRequestSchema>;
