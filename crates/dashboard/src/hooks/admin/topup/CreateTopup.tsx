import { useRef } from "react";
import useTopupStore from "@/store/topup/topup";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { CreateTopupFormValues, createTopupRequestSchema } from "@/schemas";
import { z } from "zod";
import useModalTopup from "@/store/topup/modal";
import { CreateTopup } from "@/types/domain/request/topup";

export default function useCreateTopup() {
  const { isModalVisible, showModal, hideModal } = useModalTopup();
  const formRef = useRef<HTMLFormElement>(null);

  const {
    createTopup,
    setLoadingCreateTopup,
    loadingCreateTopup,
    setErrorCreateTopup,
  } = useTopupStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: CreateTopupFormValues) => {
    setLoadingCreateTopup(true);

    try {
      const validatedValues = createTopupRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: CreateTopup = {
        card_number: validatedValues.card_number,
        topup_amount: validatedValues.topup_amount,
        topup_method: validatedValues.topup_method,
        toast: toast,
      };

      const result = await createTopup(req);

      if (result) {
        toast({
          title: "Success",
          description: "Topup berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat Topup. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateTopup(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateTopup(
          error?.message || "Terjadi kesalahan saat membuat Topup",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat membuat Topup",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateTopup(false);
      hideModal();
    }
  };

  return {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateTopup,
    isModalVisible,
    showModal,
    hideModal,
  };
}
