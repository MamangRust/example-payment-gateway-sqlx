import { UpdateRoleFormValues } from "@/schemas";

export interface RoleUpdateFormProps {
  onSubmit: (data: UpdateRoleFormValues) => void;
  defaultValues?: UpdateRoleFormValues;
}
