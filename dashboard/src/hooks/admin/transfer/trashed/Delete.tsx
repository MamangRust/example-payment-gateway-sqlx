import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";

import { DeletePermanentTransfer } from "@/types/domain/request";
import useModalTransaferTrashed from "@/store/transfer/trashed/modal";
import useTransferTrashedStore from "@/store/transfer/trashed/trashed";

export default function useDeletePermanentTransfer() {
  const {
    deletePermanentTransferId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalTransaferTrashed();

  const {
    deletePermanentTransfer,
    setLoadingDeletePermanentTransferTrashed,
    loadingDeletePermanentTransferTrashed,
    setErrorDeletePermanentTransferTrashed,
  } = useTransferTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentTransferTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentTransfer = {
        id: deletePermanentTransferId as number,
        toast: toast,
      };

      const result = await deletePermanentTransfer(req);

      if (result) {
        toast({
          title: "Success",
          description: "Transfer berhasil dihapus",
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
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Transfer";
      setErrorDeletePermanentTransferTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentTransferTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    deletePermanentTransferId,
    handleSubmit,
    loadingDeletePermanentTransferTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
