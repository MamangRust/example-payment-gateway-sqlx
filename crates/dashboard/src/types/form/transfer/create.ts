import { CreateTransferFormValues } from "@/schemas";

export interface TransferCreateFormProps {
  onSubmit: (data: CreateTransferFormValues) => void;
}
