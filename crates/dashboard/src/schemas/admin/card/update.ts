import { z } from "zod";

const UserIdSchema = z.object({
  value: z.string(),
  label: z.string(),
});

export const updateCardRequestSchema = z.object({
  user_id: UserIdSchema,
  card_type: z.string(),
  expire_date: z.coerce.date(),
  cvv: z.string(),
  card_provider: z.string(),
});

export type UpdateCardFormValues = z.infer<typeof updateCardRequestSchema>;
