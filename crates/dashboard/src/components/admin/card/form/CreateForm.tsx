import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { CreateCardFormValues, createCardRequestSchema } from "@/schemas";
import { CardCreateFormProps } from "@/types/form";
import { zodResolver } from "@hookform/resolvers/zod";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import {
  Select as ShadcnSelect,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import AsyncSelect from "react-select/async";
import useUserStore from "@/store/user/user";
import { debounce } from "@/helpers/debounce";
import { useToast } from "@/hooks/use-toast";
import { FindAllUser } from "@/types/domain/request";

const cardTypeOptions = [
  { value: "credit", label: "Credit Card" },
  { value: "debit", label: "Debit Card" },
];

const cardProviderOptions = [
  { value: "alfamart", label: "Alfamart" },
  { value: "indomart", label: "Indomart" },
  { value: "lawson", label: "Lawson" },
  { value: "dana", label: "Dana" },
  { value: "ovo", label: "OVO" },
  { value: "gopay", label: "GoPay" },
  { value: "linkaja", label: "LinkAja" },
  { value: "jenius", label: "Jenius" },
  { value: "fastpay", label: "FastPay" },
  { value: "kudo", label: "Kudo" },
  { value: "bri", label: "BRI" },
  { value: "mandiri", label: "Mandiri" },
  { value: "bca", label: "BCA" },
  { value: "bni", label: "BNI" },
  { value: "bukopin", label: "Bukopin" },
  { value: "e-banking", label: "E-Banking" },
  { value: "visa", label: "Visa" },
  { value: "mastercard", label: "MasterCard" },
  { value: "discover", label: "Discover" },
  { value: "american express", label: "American Express" },
  { value: "paypal", label: "PayPal" },
];

interface UserOption {
  value: string;
  label: string;
}

const CreateCardForm = forwardRef<HTMLFormElement, CardCreateFormProps>(
  ({ onSubmit }, ref) => {
    const {
      register,
      handleSubmit,
      control,
      formState: { errors },
    } = useForm<CreateCardFormValues>({
      resolver: zodResolver(createCardRequestSchema),
    });
    const { toast } = useToast();

    const { users, loadingGetUsers, setLoadingGetUsers, findAllUsers } =
      useUserStore();

    const [userOptions, setUserOptions] = useState<UserOption[]>([]);

    useEffect(() => {
      const fetchInitialUsers = async () => {
        setLoadingGetUsers(true);
        try {
          const searchReq: FindAllUser = {
            search: "",
            page: 1,
            page_size: 10,
            toast: toast,
          };
          await findAllUsers(searchReq);

          if (users) {
            const options = users.map((user) => ({
              value: user.id.toString(),
              label: `${user.firstname} ${user.lastname}` || user.email,
            }));
            setUserOptions(options);
          }
        } catch (error) {
          console.error("Failed to fetch initial users:", error);
        } finally {
          setLoadingGetUsers(false);
        }
      };

      fetchInitialUsers();
    }, []);

    const loadOptions = useCallback(
      debounce(
        async (
          inputValue: string,
          callback: (options: UserOption[]) => void,
        ) => {
          setLoadingGetUsers(true);
          try {
            const searchReq = {
              search: inputValue,
              page: 1,
              page_size: 10,
              toast: toast,
            };
            await findAllUsers(searchReq);

            if (users) {
              const filteredOptions = users
                .filter(
                  (user) =>
                    `${user.firstname} ${user.lastname}`
                      .toLowerCase()
                      .includes(inputValue.toLowerCase()) ||
                    user.email.toLowerCase().includes(inputValue.toLowerCase()),
                )
                .map((user) => ({
                  value: user.id.toString(),
                  label: `${user.firstname} ${user.lastname}` || user.email,
                }));

              callback(filteredOptions);
            }
          } catch (error) {
            console.error("Failed to fetch users:", error);
          } finally {
            setLoadingGetUsers(false);
          }
        },
        500,
      ),
      [users, findAllUsers, toast],
    );

    return (
      <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div>
          <Label htmlFor="userId" className="text-sm font-medium">
            User ID
          </Label>
          <Controller
            name="user_id"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                {...field}
                loadOptions={loadOptions}
                defaultOptions={userOptions}
                isLoading={loadingGetUsers}
                placeholder="Select a user"
                className="mt-1"
                styles={{
                  control: (base) => ({
                    ...base,
                    borderColor: errors.user_id ? "#ef4444" : base.borderColor,
                    borderRadius: "0.375rem",
                    padding: "0.25rem",
                    boxShadow: "none",
                    "&:hover": {
                      borderColor: "#3b82f6",
                    },
                  }),
                  option: (base, { isFocused, isSelected }) => ({
                    ...base,
                    backgroundColor: isSelected
                      ? "#3b82f6"
                      : isFocused
                        ? "#bfdbfe"
                        : "white",
                    color: isSelected ? "white" : "black",
                    "&:hover": {
                      backgroundColor: "#bfdbfe",
                    },
                  }),
                  menu: (base) => ({
                    ...base,
                    borderRadius: "0.375rem",
                    boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                  }),
                }}
                value={userOptions.find(
                  (option) => Number(option.value) === field.value,
                )}
                onChange={(selectedOption) => {
                  field.onChange(
                    selectedOption ? Number(selectedOption.value) : null,
                  );
                }}
              />
            )}
          />
          {errors.user_id && (
            <p className="text-red-500 text-sm mt-1">
              {errors.user_id.message}
            </p>
          )}
        </div>

        <div>
          <Label htmlFor="cardType" className="text-sm font-medium">
            Card Type
          </Label>
          <Controller
            name="card_type"
            control={control}
            render={({ field }) => (
              <ShadcnSelect onValueChange={field.onChange} value={field.value}>
                <SelectTrigger className="w-full mt-1">
                  <SelectValue placeholder="Select card type" />
                </SelectTrigger>
                <SelectContent>
                  {cardTypeOptions.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </ShadcnSelect>
            )}
          />
          {errors.card_type && (
            <p className="text-red-500 text-sm mt-1">
              {errors.card_type.message}
            </p>
          )}
        </div>

        <div>
          <Label htmlFor="cardProvider" className="text-sm font-medium">
            Card Provider
          </Label>
          <Controller
            name="card_provider"
            control={control}
            render={({ field }) => (
              <ShadcnSelect onValueChange={field.onChange} value={field.value}>
                <SelectTrigger className="w-full mt-1">
                  <SelectValue placeholder="Select card provider" />
                </SelectTrigger>
                <SelectContent>
                  {cardProviderOptions.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </ShadcnSelect>
            )}
          />
          {errors.card_provider && (
            <p className="text-red-500 text-sm mt-1">
              {errors.card_provider.message}
            </p>
          )}
        </div>

        <div>
          <Label htmlFor="expireDate" className="text-sm font-medium">
            Expire Date
          </Label>
          <Input
            id="expireDate"
            type="date"
            placeholder="MM/YY"
            className="mt-1"
            {...register("expire_date")}
          />
          {errors.expire_date && (
            <p className="text-red-500 text-sm mt-1">
              {errors.expire_date.message}
            </p>
          )}
        </div>

        <div>
          <Label htmlFor="cvv" className="text-sm font-medium">
            CVV
          </Label>
          <Input
            id="cvv"
            type="password"
            placeholder="Enter CVV"
            className="mt-1"
            {...register("cvv")}
          />
          {errors.cvv && (
            <p className="text-red-500 text-sm mt-1">{errors.cvv.message}</p>
          )}
        </div>
      </form>
    );
  },
);

export default CreateCardForm;
