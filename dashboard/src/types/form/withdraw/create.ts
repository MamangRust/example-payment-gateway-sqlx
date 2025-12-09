import { CreateWithdrawFormValues } from "@/schemas";

export interface WithdrawCreateFormProps {
  onSubmit: (data: CreateWithdrawFormValues) => void;
}
