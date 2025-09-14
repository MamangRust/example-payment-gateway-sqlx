import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { LoginFormValues, loginSchema } from "@/schemas/auth/login";
import { LoginFormProps } from "@/types/form/auth/login";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";

export default function LoginForm({
  handleSubmit,
  loadingLogin,
}: LoginFormProps) {
  const {
    register,
    handleSubmit: handleFormSubmit,
    formState: { errors, isValid },
  } = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
    mode: "onChange",
  });

  return (
    <form onSubmit={handleFormSubmit(handleSubmit)} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="email">Email</Label>
        <Input
          id="email"
          type="email"
          placeholder="m@example.com"
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
          {...register("password")}
          required
        />
        {errors.password && (
          <p className="text-sm text-red-500">{errors.password.message}</p>
        )}
      </div>
      <Button
        type="submit"
        className="w-full"
        disabled={!isValid || loadingLogin}
      >
        {loadingLogin ? (
          <div className="flex items-center gap-2">
            <Loader2 className="animate-spin" />
            Login...
          </div>
        ) : (
          "Login"
        )}
      </Button>
    </form>
  );
}
