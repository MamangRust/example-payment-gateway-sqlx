import useTransferStore from "@/store/transfer/transfer";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { z } from "zod";
import useModalTransfer from "@/store/transfer/modal";
import { TrashedTransfer } from "@/types/domain/request";

export default function useDeleteTransfer() {
  const {
    deleteTransferId,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useModalTransfer();

  const {
    trashedTransfer,
    setLoadingTrashedTransfer,
    loadingTrashedTransfer,
    setErrorTrashedTransfer,
  } = useTransferStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingTrashedTransfer(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: TrashedTransfer = {
        id: deleteTransferId as number,
        toast: toast,
      };

      const result = await trashedTransfer(req);

      if (result) {
        toast({
          title: "Success",
          description: "Transfer berhasil didelete",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Transfer. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorTrashedTransfer(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorTrashedTransfer(
          error?.message || "Terjadi kesalahan saat menghapus Transfer",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat menghapus Transfer",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingTrashedTransfer(false);
      hideModalDelete();
    }
  };

  return {
    deleteTransferId,
    handleSubmit,
    loadingTrashedTransfer,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  };
}
