import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { RegisterFormValues, registerSchema } from "@/schemas/auth/register";
import { RegisterFormProps } from "@/types/form/auth/register";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";

export default function RegisterForm({
  handleSubmit,
  loadingRegister,
}: RegisterFormProps) {
  const {
    register,
    handleSubmit: handleFormSubmit,
    formState: { errors, isValid },
  } = useForm<RegisterFormValues>({
    resolver: zodResolver(registerSchema),
    mode: "onChange",
  });

  return (
    <form onSubmit={handleFormSubmit(handleSubmit)} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="firstname">First Name</Label>
        <Input
          id="firstname"
          type="text"
          placeholder="Enter your first name"
          {...register("firstname")}
          required
        />
        {errors.firstname && (
          <p className="text-sm text-red-500">{errors.firstname.message}</p>
        )}
      </div>
      <div className="space-y-2">
        <Label htmlFor="lastname">Last Name</Label>
        <Input
          id="lastname"
          type="text"
          placeholder="Enter your last name"
          {...register("lastname")}
          required
        />
        {errors.lastname && (
          <p className="text-sm text-red-500">{errors.lastname.message}</p>
        )}
      </div>
      <div className="space-y-2">
        <Label htmlFor="email">Email</Label>
        <Input
          id="email"
          type="email"
          placeholder="Enter your email"
          {...register("email")}
          required
        />
        {errors.email && (
          <p className="text-sm text-red-500">{errors.email.message}</p>
        )}
      </div>
      <div className="space-y-2">
        <Label htmlFor="password">Password</Label>
        <Input
          id="password"
          type="password"
          placeholder="Enter your password"
          {...register("password")}
          required
        />
        {errors.password && (
          <p className="text-sm text-red-500">{errors.password.message}</p>
        )}
      </div>
      <div className="space-y-2">
        <Label htmlFor="confirm-password">Confirm Password</Label>
        <Input
          id="confirm-password"
          type="password"
          placeholder="Confirm your password"
          {...register("confirm_password")}
          required
        />
        {errors.confirm_password && (
          <p className="text-sm text-red-500">
            {errors.confirm_password.message}
          </p>
        )}
      </div>
      <Button
        type="submit"
        className="w-full mt-4"
        disabled={!isValid || loadingRegister}
      >
        {loadingRegister ? (
          <div className="flex items-center gap-2">
            <Loader2 className="animate-spin" />
            Register...
          </div>
        ) : (
          "Register"
        )}
      </Button>
    </form>
  );
}
