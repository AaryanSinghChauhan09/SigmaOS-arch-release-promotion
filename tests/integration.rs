use sigmaos_arch_release_promotion::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_arch_release_promotion::new("sigmaos-arch-release-promotion").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-arch-release-promotion"));
}
