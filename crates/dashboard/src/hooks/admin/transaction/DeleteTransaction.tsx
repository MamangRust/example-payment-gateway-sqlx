import useTransactionStore from "@/store/transaction/transaction";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { z } from "zod";
import useModalTransaction from "@/store/transaction/modal";
import { TrashedTransaction } from "@/types/domain/request";

export default function useDeleteTransaction() {
  const {
    deleteTransactionId,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useModalTransaction();

  const {
    trashedTransaction,
    setLoadingTrashedTransaction,
    loadingTrashedTransaction,
    setErrorTrashedTransaction,
  } = useTransactionStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingTrashedTransaction(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: TrashedTransaction = {
        id: deleteTransactionId as number,
        toast: toast,
      };

      const result = await trashedTransaction(req);

      if (result) {
        toast({
          title: "Success",
          description: "Transaction berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Transaction. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorTrashedTransaction(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorTrashedTransaction(
          error?.message || "Terjadi kesalahan saat menghapus transaction",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat menghapus transaction",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingTrashedTransaction(false);
      hideModalDelete();
    }
  };

  return {
    deleteTransactionId,
    handleSubmit,
    loadingTrashedTransaction,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  };
}
