import { UpdateTopupFormValues } from "@/schemas";

export interface TopupUpdateFormProps {
  onSubmit: (data: UpdateTopupFormValues) => void;
  defaultValues?: UpdateTopupFormValues;
}
