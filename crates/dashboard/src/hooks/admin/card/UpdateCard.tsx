import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalCard from "@/store/card/modal";
import { UpdateCardFormValues, updateCardRequestSchema } from "@/schemas";
import { FindByIdCard, UpdateCard } from "@/types/domain/request";
import useCardStore from "@/store/card/card";
import { z } from "zod";
import { useEffect, useRef } from "react";

export default function useUpdateCard() {
  const { isModalVisibleEdit, showModalEdit, hideModalEdit, editCardId } =
    useModalCard();

  const formRef = useRef<HTMLFormElement>(null);

  const {
    card,
    findByIdCard,
    updateCard,
    setLoadingUpdateCard,
    loadingUpdateCard,
    setErrorUpdateCard,
  } = useCardStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  useEffect(() => {
    if (isModalVisibleEdit && editCardId !== null) {
      const req: FindByIdCard = {
        toast,
        id: editCardId,
      };

      findByIdCard(req);
    }
  }, [isModalVisibleEdit, editCardId, findByIdCard]);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: UpdateCardFormValues) => {
    setLoadingUpdateCard(true);

    try {
      const validatedValues = updateCardRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: UpdateCard = {
        card_id: editCardId as number,
        user_id: Number(validatedValues.user_id.value),
        card_type: validatedValues.card_type,
        expire_date: validatedValues.expire_date,
        cvv: validatedValues.cvv,
        card_provider: validatedValues.card_provider,
        toast: toast,
      };
      console.log("hello", req);

      const result = await updateCard(req);

      if (result) {
        toast({
          title: "Success",
          description: "User berhasil diupdate",
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
        setErrorUpdateCard(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorUpdateCard(
          error?.message || "Terjadi kesalahan saat mengedit user",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat mengedit user",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingUpdateCard(false);
      hideModalEdit();
    }
  };

  return {
    card,
    handleSubmit,
    editCardId,
    loadingUpdateCard,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
    formRef,
    handleButtonSubmit,
  };
}
