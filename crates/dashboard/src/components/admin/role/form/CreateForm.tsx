import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { CreateRoleFormValues, createRoleRequestSchema } from "@/schemas";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { forwardRef } from "react";
import { RoleCreateFormProps } from "@/types/form/role/create";

const CreateRoleForm = forwardRef<HTMLFormElement, RoleCreateFormProps>(
  ({ onSubmit }, ref) => {
    const {
      register,
      handleSubmit,
      formState: { errors },
    } = useForm<CreateRoleFormValues>({
      resolver: zodResolver(createRoleRequestSchema),
    });

    return (
      <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div>
          <Label htmlFor="firstname" className="text-sm font-medium">
            First Name
          </Label>
          <Input
            id="firstname"
            type="text"
            placeholder="Enter first name"
            className="mt-1"
            {...register("name")}
          />
          {errors.name && (
            <p className="text-sm text-red-500">{errors.name.message}</p>
          )}
        </div>
      </form>
    );
  },
);

CreateRoleForm.displayName = "CreateRoleForm";

export default CreateRoleForm;
