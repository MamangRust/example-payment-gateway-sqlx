import { UpdateCardFormValues } from "@/schemas";

export interface CardUpdateFormProps {
  onSubmit: (data: UpdateCardFormValues) => void;
  defaultValues?: UpdateCardFormValues;
}
