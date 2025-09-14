import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { forwardRef } from "react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { UpdateRoleFormValues, updateRoleRequestSchema } from "@/schemas";

interface RoleUpdateFormProps {
  onSubmit: (data: UpdateRoleFormValues) => void;
  defaultValues?: UpdateRoleFormValues;
}

const UpdateRoleForm = forwardRef<HTMLFormElement, RoleUpdateFormProps>(
  ({ onSubmit, defaultValues }, ref) => {
    const {
      register,
      handleSubmit,
      formState: { errors },
    } = useForm<UpdateRoleFormValues>({
      resolver: zodResolver(updateRoleRequestSchema),
      defaultValues: defaultValues || {
        name: "",
      },
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

UpdateRoleForm.displayName = "UpdateRoleForm";

export default UpdateRoleForm;
