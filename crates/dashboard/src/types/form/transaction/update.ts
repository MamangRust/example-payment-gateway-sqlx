import { UpdateTransactionFormValues } from "@/schemas";

export interface TransactionUpdateFormProps {
  onSubmit: (data: UpdateTransactionFormValues) => void;
  defaultValues?: UpdateTransactionFormValues;
}
