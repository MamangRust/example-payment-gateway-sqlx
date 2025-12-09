import { UpdateSaldoFormValues } from "@/schemas";

export interface SaldoUpdateFormProps {
  onSubmit: (data: UpdateSaldoFormValues) => void;
  defaultValues?: UpdateSaldoFormValues;
}
