import useTopupStore from "@/store/topup/topup";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { z } from "zod";
import useModalTopup from "@/store/topup/modal";
import { TrashedTopup } from "@/types/domain/request/topup";

export default function useDeleteTopup() {
  const {
    deleteTopupId,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useModalTopup();

  const {
    trashedTopup,
    setLoadingTrashedTopup,
    loadingTrashedTopup,
    setErrorTrashedTopup,
  } = useTopupStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingTrashedTopup(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: TrashedTopup = {
        id: deleteTopupId as number,
        toast: toast,
      };

      const result = await trashedTopup(req);

      if (result) {
        toast({
          title: "Success",
          description: "Topup berhasil didelete",
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
        setErrorTrashedTopup(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorTrashedTopup(
          error?.message || "Terjadi kesalahan saat mengedit topup",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat mengedit topup",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingTrashedTopup(false);
      hideModalDelete();
    }
  };

  return {
    deleteTopupId,
    handleSubmit,
    loadingTrashedTopup,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  };
}
