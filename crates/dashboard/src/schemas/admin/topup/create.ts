import { z } from "zod";

export const createTopupRequestSchema = z.object({
  card_number: z.string().min(1).max(16),
  topup_amount: z.number().min(1).max(16),
  topup_method: z.string().min(1).max(16),
});

export type CreateTopupFormValues = z.infer<typeof createTopupRequestSchema>;
