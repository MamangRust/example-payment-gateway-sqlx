import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTopupTrashed from "@/store/topup/trashed/modal";
import useTopupTrashedStore from "@/store/topup/trashed/trashed";
import { DeletePermanentTopup } from "@/types/domain/request/topup";

export default function useDeletePermanentTopup() {
  const {
    deletePermanentTopupId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalTopupTrashed();

  const {
    deletePermanentTopup,
    setLoadingDeletePermanentTopupTrashed,
    loadingDeletePermanentTopupTrashed,
    setErrorDeletePermanentTopupTrashed,
  } = useTopupTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentTopupTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentTopup = {
        id: deletePermanentTopupId as number,
        toast: toast,
      };

      const result = await deletePermanentTopup(req);

      if (result) {
        toast({
          title: "Success",
          description: "Topup berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Topup. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Topup";
      setErrorDeletePermanentTopupTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentTopupTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    deletePermanentTopupId,
    handleSubmit,
    loadingDeletePermanentTopupTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
