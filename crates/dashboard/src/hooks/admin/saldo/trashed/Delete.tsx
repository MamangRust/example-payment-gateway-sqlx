import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalSaldoTrashed from "@/store/saldo/trashed/modal";
import useSaldoTrashedStore from "@/store/saldo/trashed/trashed";
import { DeletePermanentSaldo } from "@/types/domain/request";

export default function useDeletePermanentSaldo() {
  const {
    deletePermanentSaldoId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalSaldoTrashed();

  const {
    deletePermanentSaldo,
    setLoadingDeletePermanentSaldoTrashed,
    loadingDeletePermanentSaldoTrashed,
    setErrorDeletePermanentSaldoTrashed,
  } = useSaldoTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentSaldoTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentSaldo = {
        id: deletePermanentSaldoId as number,
        toast: toast,
      };

      const result = await deletePermanentSaldo(req);

      if (result) {
        toast({
          title: "Success",
          description: "Saldo berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Saldo. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Saldo";
      setErrorDeletePermanentSaldoTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentSaldoTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    deletePermanentSaldoId,
    handleSubmit,
    loadingDeletePermanentSaldoTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
