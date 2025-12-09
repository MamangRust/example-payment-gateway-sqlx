import { CreateTopupFormValues } from "@/schemas";

export interface TopupCreateFormProps {
  onSubmit: (data: CreateTopupFormValues) => void;
}
