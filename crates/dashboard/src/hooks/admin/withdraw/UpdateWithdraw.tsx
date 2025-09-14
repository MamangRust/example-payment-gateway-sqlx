import { useEffect, useRef } from "react";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import {
  UpdateWithdrawFormValues,
  updateWithdrawRequestSchema,
} from "@/schemas";
import { z } from "zod";
import useModalWithdraw from "@/store/withdraw/modal";
import { FindByIdWithdraw, UpdateWithdraw } from "@/types/domain/request";

export default function useUpdateWithdraw() {
  const { isModalVisibleEdit, showModalEdit, hideModalEdit, editWithdrawId } =
    useModalWithdraw();

  const {
    updateWithdraw,
    setLoadingUpdateWithdraw,
    loadingUpdateWithdraw,
    setErrorUpdateWithdraw,

    withdraw,
    findByIdWithdraw,
  } = useWithdrawStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const formRef = useRef<HTMLFormElement>(null);

  useEffect(() => {
    if (isModalVisibleEdit && editWithdrawId != null) {
      const req: FindByIdWithdraw = {
        toast,
        id: editWithdrawId,
      };

      findByIdWithdraw(req);
    }
  }, [isModalVisibleEdit, editWithdrawId]);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: UpdateWithdrawFormValues) => {
    setLoadingUpdateWithdraw(true);

    try {
      const validatedValues = updateWithdrawRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: UpdateWithdraw = {
        id: editWithdrawId as number,
        card_number: validatedValues.card_number.value,
        withdraw_amount: validatedValues.withdraw_amount,
        withdraw_time: validatedValues.withdraw_time,
        toast: toast,
      };

      console.log("req", req);

      const result = await updateWithdraw(req);

      if (result) {
        toast({
          title: "Success",
          description: "withdraw berhasil diupdate",
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
        setErrorUpdateWithdraw(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorUpdateWithdraw(
          error?.message || "Terjadi kesalahan saat mengedit withdraw",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat mengedit withdraw",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingUpdateWithdraw(false);
      hideModalEdit();
    }
  };

  return {
    withdraw,
    formRef,
    handleButtonSubmit,
    editWithdrawId,
    handleSubmit,
    loadingUpdateWithdraw,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  };
}
