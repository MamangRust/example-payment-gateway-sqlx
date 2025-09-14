import { useRef } from "react";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import {
  CreateWithdrawFormValues,
  createWithdrawRequestSchema,
} from "@/schemas";
import { z } from "zod";
import useModalWithdraw from "@/store/withdraw/modal";
import { CreateWithdraw } from "@/types/domain/request";

export default function useCreateWithdraw() {
  const { isModalVisible, showModal, hideModal } = useModalWithdraw();

  const {
    createWithdraw,
    setLoadingCreateWithdraw,
    loadingCreateWithdraw,
    setErrorCreateWithdraw,
  } = useWithdrawStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const formRef = useRef<HTMLFormElement>(null);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: CreateWithdrawFormValues) => {
    setLoadingCreateWithdraw(true);

    try {
      const validatedValues = createWithdrawRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: CreateWithdraw = {
        card_number: validatedValues.card_number,
        withdraw_amount: validatedValues.withdraw_amount,
        withdraw_time: validatedValues.withdraw_time,
        toast: toast,
      };

      const result = await createWithdraw(req);

      if (result) {
        toast({
          title: "Success",
          description: "withdraw berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat withdraw. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateWithdraw(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateWithdraw(
          error?.message || "Terjadi kesalahan saat membuat withdraw",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat membuat withdraw",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateWithdraw(false);
      hideModal();
    }
  };

  return {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateWithdraw,
    isModalVisible,
    showModal,
    hideModal,
  };
}
