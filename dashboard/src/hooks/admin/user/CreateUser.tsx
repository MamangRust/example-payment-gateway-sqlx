import { useRef } from "react";
import useUserStore from "@/store/user/user";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { CreateUserFormValues, createUserRequestSchema } from "@/schemas";
import { z } from "zod";
import useModalUser from "@/store/user/modal";
import { CreateUser } from "@/types/domain/request";

export default function useCreateUser() {
  const { isModalVisible, showModal, hideModal } = useModalUser();

  const {
    createUser,
    setLoadingCreateUser,
    loadingCreateUser,
    setErrorCreateUser,
  } = useUserStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const formRef = useRef<HTMLFormElement>(null);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: CreateUserFormValues) => {
    setLoadingCreateUser(true);

    try {
      const validatedValues = createUserRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: CreateUser = {
        firstname: validatedValues.firstname,
        lastname: validatedValues.lastname,
        email: validatedValues.email,
        password: validatedValues.password,
        confirm_password: validatedValues.confirm_password,
        toast: toast,
      };

      const result = await createUser(req);

      if (result) {
        toast({
          title: "Success",
          description: "User berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat user. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateUser(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateUser(
          error?.message || "Terjadi kesalahan saat membuat user",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat membuat user",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateUser(false);
      hideModal();
    }
  };

  return {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateUser,
    isModalVisible,
    showModal,
    hideModal,
  };
}
