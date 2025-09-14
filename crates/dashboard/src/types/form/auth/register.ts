import { RegisterFormValues } from "@/schemas/auth/register";
import { SubmitHandler } from "react-hook-form";

export interface RegisterFormProps {
  handleSubmit: SubmitHandler<RegisterFormValues>;
  loadingRegister: boolean;
}
