import { z } from "zod";

const CardIdSchema = z.object({
  value: z.string(),
  label: z.string(),
});

export const updateSaldoRequestSchema = z.object({
  card_number: CardIdSchema,
  total_balance: z.number().min(1).max(100000000),
});

export type UpdateSaldoFormValues = z.infer<typeof updateSaldoRequestSchema>;
