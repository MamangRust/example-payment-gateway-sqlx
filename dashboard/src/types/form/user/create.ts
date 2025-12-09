import { CreateUserFormValues } from "@/schemas";

export interface UserCreateFormProps {
  onSubmit: (data: CreateUserFormValues) => void;
}
