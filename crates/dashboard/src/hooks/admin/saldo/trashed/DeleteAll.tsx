import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalSaldoTrashed from "@/store/saldo/trashed/modal";
import useSaldoTrashedStore from "@/store/saldo/trashed/trashed";

export default function useDeletePermanentAllSaldo() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalSaldoTrashed();

  const {
    deletePermanentAllSaldo,
    setLoadingDeletePermanentAllSaldoTrashed,
    loadingDeletePermanentAllSaldoTrashed,
    setErrorDeletePermanentAllSaldoTrashed,
  } = useSaldoTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllSaldoTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllSaldo(toast);

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
      setErrorDeletePermanentAllSaldoTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllSaldoTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllSaldoTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
