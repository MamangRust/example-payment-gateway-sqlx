import { z } from "zod";


export const createTransferRequestSchema = z.object({
  transfer_from: z.string().min(1).max(16),
  transfer_to: z.string().min(1).max(16),
  transfer_amount: z.number().min(1).max(100000000),
});

export type CreateTransferFormValues = z.infer<
  typeof createTransferRequestSchema
>;
