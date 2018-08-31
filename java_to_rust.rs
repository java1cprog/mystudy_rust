/*
////////////////////////////////////////////////////////////////
class ArrayApp {
    public static void main(String[] args) {
        long[] arr; // Ссылка на массив
        arr = new long[100]; // Создание массива
        int nElems = 0; // Количество элементов
        int j; // Счетчик цикла
        long searchKey; // Ключи искомого элемента
        //--------------------------------------------------------------
        arr[0] = 77; // Вставка 10 элементов
        arr[1] = 99;
        arr[2] = 44;
        arr[3] = 55;
        arr[4] = 22;
        arr[5] = 88;
        arr[6] = 11;
        arr[7] = 00;
        arr[8] = 66;
        arr[9] = 33;
        nElems = 10; // Массив содержит 10 элементов
        //--------------------------------------------------------------
        for (j = 0; j < nElems; j++) // Вывод элементов
            System.out.print(arr[j] + " ");
        System.out.println("");
        //--------------------------------------------------------------
        searchKey = 66; // Поиск элемента с ключом 66
        for (j = 0; j < nElems; j++) // Для каждого элемента
            if (arr[j] == searchKey) // Значение найдено?
                break; // Да - выход из цикла
        if (j == nElems) // Достигнут последний элемент?
            System.out.println("Can't find " + searchKey); // Да
        else System.out.println("Found " + searchKey); // Нет
        //--------------------------------------------------------------
        searchKey = 55; // Удаление элемента с ключом 55
        for (j = 0; j < nElems; j++) // Поиск удаляемого элемента
            if (arr[j] == searchKey) break;
        for (int k = j; k < nElems - 1; k++) // Сдвиг последующих элементов
            arr[k] = arr[k + 1];
        nElems--; // Уменьшение размера
        //--------------------------------------------------------------
        for (j = 0; j < nElems; j++) // Вывод элементов
            System.out.print(arr[j] + " ");
        System.out.println("");
    }
} // Конец класса ArrayApp


*/

fn main() {
    let mut arr: [u32; 100] = [0; 100];
    arr[0] = 77; // Вставка 10 элементов
    arr[1] = 99;
    arr[2] = 44;
    arr[3] = 55;
    arr[4] = 22;
    arr[5] = 88;
    arr[6] = 11;
    arr[7] = 00;
    arr[8] = 66;
    arr[9] = 33;

    let n_elems: u8 = 10; // Массив содержит 10 элементов

    //--------------------------------------------------------------

    for (i, element) in arr.iter().enumerate() {
        // Вывод элементов
        println!("{}) {} ", i, element);
        if i as u8 + 1 == n_elems {
            break;
        }
    }
    //--------------------------------------------------------------

    let search_key: u8 = 66;
    for (i, element) in arr.iter().enumerate() {
        if element == search_key {
            break;
        }

        if i == n_elems {
            println!("Can't find {}", search_key);
        } else {
            println!("Found {}", search_key);
        }
    }
    //--------------------------------------------------------------
}
