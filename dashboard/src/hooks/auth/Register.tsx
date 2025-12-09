import useAuthStore from "@/store/auth";
import { useToast } from "../use-toast";
import { useNavigate } from "react-router-dom";
import { SubmitHandler } from "react-hook-form";
import { LoginFormValues } from "@/schemas/auth/login";
import { registerSchema } from "@/schemas/auth/register";
import { z } from "zod";

export default function useRegister() {
  const navigate = useNavigate();

  const { register, setLoadingRegister, setErrorRegister, loadingRegister } =
    useAuthStore();
  const { toast } = useToast();

  const onFinish: SubmitHandler<LoginFormValues> = async (data) => {
    setLoadingRegister(true);

    try {
      const validatedValues = registerSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const response = await register(validatedValues, toast);

      if (response) {
        toast({
          title: "Success",
          description: "Register Berhasil",
          variant: "default",
        });
        navigate("/auth/login");
      } else {
        toast({
          title: "Error",
          description: "Register gagal. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorRegister(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorRegister(error?.message || "Terjadi kesalahan saat register");
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat register",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingRegister(false);
    }
  };

  return {
    onFinish,
    loadingRegister,
  };
}
