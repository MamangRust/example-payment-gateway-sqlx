import { UpdateTransferFormValues } from "@/schemas";

export interface TransferUpdateFormProps {
  onSubmit: (data: UpdateTransferFormValues) => void;
  defaultValues?: UpdateTransferFormValues;
}
