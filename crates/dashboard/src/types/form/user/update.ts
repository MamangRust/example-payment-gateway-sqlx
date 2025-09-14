import { UpdateUserFormValues } from "@/schemas";

export interface UserUpdateFormProps {
  onSubmit: (data: UpdateUserFormValues) => void;
}
