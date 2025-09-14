import { z } from "zod";

const TransferFromSchema = z.object({
  value: z.string(),
  label: z.string(),
});

const TransferToSchema = z.object({
  value: z.string(),
  label: z.string(),
});

export const updateTransferRequestSchema = z.object({
  transfer_from: TransferFromSchema,
  transfer_to: TransferToSchema,
  transfer_amount: z.number().min(1).max(100000000),
});

export type UpdateTransferFormValues = z.infer<
  typeof updateTransferRequestSchema
>;
