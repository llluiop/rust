#lifetimes

##生命周期

* 我获取了一个某种资源的句柄
* 我借给你了一个关于这个资源的引用
* 我决定不再需要这个资源了，然后释放了它，这时你仍然持有它的引用
* 你决定使用这个资源  //**opps，出错了**

##显示的管理生命周期

当我们有一个获取引用作为参数的函数，我们可以隐式或显式涉及到引用的生命周期：

    // implicit
    fn foo(x: &i32) {
    }

    // explicit
    fn foo<'a>(x: &'a i32) {
    }
    
    'a读作“生命周期 a”, 神奇的语法