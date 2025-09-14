import useCardStore from "@/store/card/card";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { CreateCardFormValues, createCardRequestSchema } from "@/schemas";
import { z } from "zod";
import useModalCard from "@/store/card/modal";
import { CreateCard } from "@/types/domain/request";
import { useRef } from "react";

export default function useCreateCard() {
  const { isModalVisible, showModal, hideModal } = useModalCard();
  const formRef = useRef<HTMLFormElement>(null);

  const {
    createCard,
    setLoadingCreateCard,
    loadingCreateCard,
    setErrorCreateCard,
  } = useCardStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async (data: CreateCardFormValues) => {
    setLoadingCreateCard(true);

    try {
      const validatedValues = createCardRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: CreateCard = {
        user_id: Number(validatedValues.user_id),
        card_type: validatedValues.card_type,
        expire_date: new Date(validatedValues.expire_date.toISOString()),
        cvv: validatedValues.cvv,
        card_provider: validatedValues.card_provider,
        toast: toast,
      };
      console.log("req", req);

      const result = await createCard(req);

      if (result) {
        toast({
          title: "Success",
          description: "Card berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat Card. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateCard(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateCard(
          error?.message || "Terjadi kesalahan saat membuat Card",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat membuat Card",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateCard(false);
      hideModal();
    }
  };

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  return {
    handleSubmit,
    loadingCreateCard,
    isModalVisible,
    showModal,
    hideModal,
    formRef,
    handleButtonSubmit,
  };
}
