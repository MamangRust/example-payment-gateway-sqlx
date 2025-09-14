import { CreateRoleFormValues } from "@/schemas";

export interface RoleCreateFormProps {
  onSubmit: (data: CreateRoleFormValues) => void;
}
