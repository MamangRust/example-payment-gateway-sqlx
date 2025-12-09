import useAuthStore from "@/store/auth";
import { useToast } from "../use-toast";
import { useNavigate } from "react-router-dom";
import { LoginFormValues, loginSchema } from "@/schemas/auth/login";
import { z } from "zod";
import { SubmitHandler } from "react-hook-form";

export default function useLogin() {
  const { login, setLoadingLogin, loadingLogin, setErrorLogin } =
    useAuthStore();
  const navigate = useNavigate();
  const { toast } = useToast();

  const onFinish: SubmitHandler<LoginFormValues> = async (data) => {
    setLoadingLogin(true);

    try {
      const validatedValues = loginSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const loginResult = await login(validatedValues, toast);

      if (loginResult) {
        toast({
          title: "Success",
          description: "Login Berhasil",
          variant: "default",
        });

        navigate("/dashboard/saldo");
      } else {
        toast({
          title: "Error",
          description: "Login gagal. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorLogin(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorLogin(error?.message || "Terjadi kesalahan saat login");
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat login",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingLogin(false);
    }
  };

  return {
    onFinish,
    loadingLogin,
  };
}
