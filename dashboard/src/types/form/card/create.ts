import { CreateCardFormValues } from "@/schemas";

export interface CardCreateFormProps {
  onSubmit: (data: CreateCardFormValues) => void;
}
