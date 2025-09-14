import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback, useMemo } from "react";


export default function useYearAmountMerchantDetail(toast: any) {
    const {
        findYearAmountByMerchant,
        yearAmount,
        loadingYearAmount,
        setLoadingYearAmount,
        setErrorYearAmount,
    } = useMerchantStore();

    const yearlyAmount = useMemo(() => {
        if (!yearAmount || !Array.isArray(yearAmount)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearAmount.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearAmount]);

    const fetchYearlyAmount = useCallback(
        async (year: number, merchant_id: number) => {
            try {
                setLoadingYearAmount(true);
                setErrorYearAmount(null);

                await findYearAmountByMerchant(toast, year, merchant_id);
            } catch (error) {
                setErrorYearAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearAmount(false);
            }
        },
        [findYearAmountByMerchant, setLoadingYearAmount, setErrorYearAmount],
    );

    const debouncedFetchYearlyAmount = debounce(fetchYearlyAmount, 1000);

    return {
        debouncedFetchYearlyAmount,
        findYearAmountByMerchant,
        yearAmount,
        loadingYearAmount,
        setLoadingYearAmount,
        setErrorYearAmount,
        yearlyAmount,
        fetchYearlyAmount
    }

}