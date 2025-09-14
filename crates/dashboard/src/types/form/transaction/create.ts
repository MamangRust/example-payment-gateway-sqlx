import { CreateTransactionFormValues } from "@/schemas";

export interface TransactionCreateFormProps {
  onSubmit: (data: CreateTransactionFormValues) => void;
}
