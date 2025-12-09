import { z } from "zod";

export const createSaldoRequestSchema = z.object({
  card_number: z.string().min(1).max(16),
  total_balance: z.number().min(1).max(100000000),
});

export type CreateSaldoFormValues = z.infer<typeof createSaldoRequestSchema>;
