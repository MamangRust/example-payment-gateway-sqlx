import { z } from "zod";

export const createCardRequestSchema = z.object({
  user_id: z.number().int().min(1),
  card_type: z.string(),
  expire_date: z.coerce.date(),
  cvv: z.string(),
  card_provider: z.string(),
});

export type CreateCardFormValues = z.infer<typeof createCardRequestSchema>;
