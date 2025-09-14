import { UpdateWithdrawFormValues } from "@/schemas";

export interface WithdrawUpdateFormProps {
  onSubmit: (data: UpdateWithdrawFormValues) => void;
  defaultValues?: UpdateWithdrawFormValues;
}
